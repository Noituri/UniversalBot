package main

import (
	"context"
	"github.com/aws/aws-lambda-go/lambda"
)

type TestEvent struct {
	Message string `json:"message"`
}

func handle(ctx context.Context, event TestEvent) (string, error) {
	return "You said: " + event.Message, nil
}

func main() {
	lambda.Start(handle)
}
