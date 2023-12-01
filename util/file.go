package util

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
)

func GetBufReader(relFilePath string) (*bufio.Reader, error) {
	filePath, err := filepath.Abs(relFilePath)

	if err != nil {
		return nil, fmt.Errorf("failed to get absolute path for the provide relative file path due to %w", err)
	}

	file, err := os.Open(filePath)

	if err != nil {
		return nil, fmt.Errorf("failed to open the file: %v due to %w", relFilePath, err)
	}

	defer file.Close()

	reader := bufio.NewReader(file)

	return reader, nil
}