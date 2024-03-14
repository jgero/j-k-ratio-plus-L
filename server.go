package main

import (
	"bytes"
	"context"
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/a-h/templ"
	"github.com/labstack/echo/v4"
	"github.com/labstack/gommon/log"
)

type Server struct {
	ctx context.Context
	app *echo.Echo
	sb  *Scoreboard
	ckw *CompileKotlinWorker
}

func NewServer(ctx context.Context, ckw *CompileKotlinWorker) (server *Server) {
	server = &Server{
		app: echo.New(),
		ctx: ctx,
		sb:  NewScoreboard(ctx),
		ckw: ckw,
	}
	server.app.GET("/", server.homeHandler)
	server.app.POST("/compile", server.compileKotlinHandler)
	server.app.GET("/scoreboard", server.scoreboardHandler)
	server.app.Static("css", "./css")
	server.app.Logger.SetLevel(log.DEBUG)
	return
}

func (server *Server) Start() {
	go func() {
		if err := server.app.Start(":4000"); err != nil {
			server.app.Logger.Fatal(err)
		}
		server.app.Logger.Info("Server ready")
	}()
	<-server.ctx.Done()
	server.app.Logger.Info("Shutting down")
	shutdownGracePeriod, cancel := context.WithTimeout(context.Background(), time.Second*5)
	err := server.app.Shutdown(shutdownGracePeriod)
	if err != nil {
		println(err)
	}
	server.app.Logger.Info("Shutdown complete")
	cancel()
}

func (server *Server) homeHandler(c echo.Context) error {
	return render(c, http.StatusOK, doc())
}

func (server *Server) compileKotlinHandler(c echo.Context) error {
	kotlin := c.FormValue("code")
	javaFiles, err := server.ckw.compute(kotlin)
	if err != nil {
		c.Response().Header().Set(echo.HeaderContentType, echo.MIMETextHTML)
		_, wErr := c.Response().Write([]byte(fmt.Sprintf("<div id=\"editor-java\"><pre>%s</pre></div>", err)))
		return wErr
	}
	java := strings.Join(javaFiles, "\n")
	lr := (strings.Count(java, "\n") + 1) / (strings.Count(kotlin, "\n") + 1)
	cr := len(java) / len(kotlin)
	server.sb.Register("user", CompressionRaio{line: lr, character: cr})
	return render(c, http.StatusOK, editor("java", java, ""))
}

func (server *Server) scoreboardHandler(c echo.Context) error {
	c.Response().Header().Set(echo.HeaderContentType, "text/event-stream")
	c.Response().Header().Set(echo.HeaderCacheControl, "no-cache")
	c.Response().Header().Set("Connection", "keep-alive")
	var data ScoreboardData
	var buf bytes.Buffer
	data = server.sb.Get()
	writeResponse(data, c, buf)
	receiver := make(chan ScoreboardData)
	server.sb.Subscribe(receiver)
	for {
		select {
		case data = <-receiver:
			writeResponse(data, c, buf)
		case <-c.Request().Context().Done():
			server.sb.Unsubscribe(receiver)
			return nil
		}
	}
}

func writeResponse(data ScoreboardData, c echo.Context, buf bytes.Buffer) error {
	if err := scoreboard(data).Render(c.Request().Context(), &buf); err != nil {
		return err
	}
	fmt.Fprintf(c.Response().Writer, "event: ScoreboardUpdate\ndata: %s\n\n", buf.String())
	c.Response().Flush()
	return nil
}

func render(ctx echo.Context, statusCode int, t templ.Component) error {
	ctx.Response().Writer.WriteHeader(statusCode)
	ctx.Response().Header().Set(echo.HeaderContentType, echo.MIMETextHTML)
	return t.Render(ctx.Request().Context(), ctx.Response().Writer)
}
