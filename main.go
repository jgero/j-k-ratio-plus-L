package main

import (
	"fmt"
	"net/http"
	"time"

	"github.com/a-h/templ"
	"github.com/labstack/echo/v4"
)

func main() {
	app := echo.New()
	app.GET("/", HomeHandler)
	app.GET("/scoreboard", ScoreboardHandler)
	app.Logger.Fatal(app.Start(":4000"))
}

// This custom Render replaces Echo's echo.Context.Render() with templ's templ.Component.Render().
func Render(ctx echo.Context, statusCode int, t templ.Component) error {
	ctx.Response().Writer.WriteHeader(statusCode)
	ctx.Response().Header().Set(echo.HeaderContentType, echo.MIMETextHTML)
	return t.Render(ctx.Request().Context(), ctx.Response().Writer)
}

func ScoreboardHandler(c echo.Context) error {
	c.Response().Header().Set(echo.HeaderContentType, "text/event-stream")
	c.Response().Header().Set(echo.HeaderCacheControl, "no-cache")
	c.Response().Header().Set("Connection", "keep-alive")
	for i := 1; i <= 10; i++ {
		fmt.Fprintf(c.Response().Writer, "event: ScoreboardUpdate\ndata: <div>hello world #%d<div>\n\n", i)
		c.Response().Flush()
		time.Sleep(1 * time.Second)
	}
	select {
		case <- c.Request().Context().Done():
			return nil
	}
}

func HomeHandler(c echo.Context) error {
	return Render(c, http.StatusOK, doc())
}
