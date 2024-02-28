package main

import (
	"context"
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	var kotlinBin, jdBin string
	flag.StringVar(&kotlinBin, "kotlin-bin", "kotlin", "path to the kotlin bin")
	flag.StringVar(&jdBin, "jd-bin", "jd-cli", "path to the jd-cli bin")
	flag.Parse()
	server := NewServer(ctx, NewCompileKotlinWorker(kotlinBin, jdBin))
	server.Start()

	sigs := make(chan os.Signal, 1)
	signal.Notify(sigs, os.Interrupt, syscall.SIGINT, syscall.SIGTERM)
	<-sigs
	fmt.Println("shutting down gracefully")
	cancel()
	fmt.Println("shutdown finished, goodbye")
	os.Exit(0)
}
