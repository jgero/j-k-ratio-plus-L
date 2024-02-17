package main

import (
	"os"
	"os/exec"
	"path/filepath"
)

const IN_FILE_NAME = "in.kt"

func compileKotlin(kotlin string) (javaFiles []string, err error) {
	dir, err := os.MkdirTemp("", "j-k-ratio-compile-work-d")
	defer os.RemoveAll(dir)
	if err != nil {
		return nil, err
	}
	kotlinFilePath := filepath.Join(dir, IN_FILE_NAME)
	err = os.WriteFile(kotlinFilePath, []byte(kotlin), 0644)
	if err != nil {
		return nil, err
	}
	err = exec.Command("kotlinc", kotlinFilePath, "-d", dir).Run()
	if err != nil {
		return nil, err
	}
	err = exec.Command("jd-cli", "-od", dir, dir).Run()
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	for _, entry := range entries {
		if filepath.Ext(entry.Name()) == ".java" {
			bContent, err := os.ReadFile(filepath.Join(dir, entry.Name()))
			if err != nil {
				return nil, err
			}
			javaFiles = append(javaFiles, string(bContent))
		}
	}
	return
}
