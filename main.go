package main

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"syscall"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	server := NewServer(ctx)
	server.Start()

	sigs := make(chan os.Signal, 1)
	signal.Notify(sigs, os.Interrupt, syscall.SIGINT, syscall.SIGTERM)
	<-sigs
	fmt.Println("shutting down gracefully")
	cancel()
	fmt.Println("shutdown finished, goodbye")
	os.Exit(0)
}
