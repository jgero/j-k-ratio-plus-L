package main

import (
	"os"
	"os/exec"
	"path/filepath"
)

func NewCompileKotlinWorker(kotlinBin string, jdBin string) *CompileKotlinWorker {
	return &CompileKotlinWorker{
		inFile:    "in.kt",
		kotlinBin: kotlinBin,
		jdBin:     jdBin,
	}
}

type CompileKotlinWorker struct {
	inFile    string
	kotlinBin string
	jdBin     string
}

func (w *CompileKotlinWorker) compute(kotlin string) (javaFiles []string, err error) {
	dir, err := os.MkdirTemp("", "j-k-ratio-compile-work-d")
	defer os.RemoveAll(dir)
	if err != nil {
		return nil, err
	}
	kotlinFilePath := filepath.Join(dir, w.inFile)
	err = os.WriteFile(kotlinFilePath, []byte(kotlin), 0644)
	if err != nil {
		return nil, err
	}
	err = exec.Command(w.kotlinBin, kotlinFilePath, "-d", dir).Run()
	if err != nil {
		return nil, fmt.Errorf("%s:\n%s", err, out)
	}
	err = exec.Command(w.jdBin, "-od", dir, dir).Run()
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
