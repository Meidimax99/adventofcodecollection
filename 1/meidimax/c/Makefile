
CC := gcc
TARGET := main
INPUT := ../input.txt

all: build run clean 

run: $(TARGET)
	@cat $(INPUT) | ./$(TARGET)

build: $(TARGET).c
	@$(CC) $(TARGET).c -o $(TARGET)

clean: $(TARGET)
	@rm $(TARGET)