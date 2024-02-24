package main

import (
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
}

func NewServer(ctx context.Context) (server *Server) {
	server = &Server{
		app: echo.New(),
		ctx: ctx,
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
	javaFiles, err := compileKotlin(c.FormValue("code"))
	if err != nil {
		println(err.Error())
		c.Response().WriteHeader(http.StatusBadRequest)
		return err
	}
	return render(c, http.StatusOK, editor("java", strings.Join(javaFiles, "\n\n"), ""))
}

func (server *Server) scoreboardHandler(c echo.Context) error {
	c.Response().Header().Set(echo.HeaderContentType, "text/event-stream")
	c.Response().Header().Set(echo.HeaderCacheControl, "no-cache")
	c.Response().Header().Set("Connection", "keep-alive")
	for i := 1; i <= 10; i++ {
		fmt.Fprintf(c.Response().Writer, "event: ScoreboardUpdate\ndata: <div>hello world #%d<div>\n\n", i)
		c.Response().Flush()
		time.Sleep(1 * time.Second)
	}
	select {
	case <-c.Request().Context().Done():
		return nil
	}
}

func render(ctx echo.Context, statusCode int, t templ.Component) error {
	ctx.Response().Writer.WriteHeader(statusCode)
	ctx.Response().Header().Set(echo.HeaderContentType, echo.MIMETextHTML)
	return t.Render(ctx.Request().Context(), ctx.Response().Writer)
}
