#include <stdio.h>


typedef enum TokenEnum {
    ROCK = 0,
    PAPER = 1,
    SISSOR = 2
} Token;


typedef enum GameResultEnum {
    WIN = 2,
    DRAW = 1,
    LOSE = 0
} GameResult;


const GameResult evaluationMatrix[3][3] = {{DRAW,LOSE,WIN},{WIN,DRAW,LOSE},{LOSE,WIN,DRAW}};

//First comes opponent move, then the desired result, out comes the move to take
const Token whichTokenToPick[3][3] = {{SISSOR,ROCK,PAPER},{ROCK, PAPER, SISSOR},{PAPER,SISSOR,ROCK}};

Token symbolToToken(char symbol) {
    switch (symbol)
    {
    case 'A':
    case 'X':
        return ROCK;
        break;
    case 'B':
    case 'Y':
        return PAPER;
        break;
    case 'C':
    case 'Z':
        return SISSOR;
        break;
    }
}

GameResult symbolToResult(char symbol) {
    switch (symbol)
    {
    case 'X':
        return LOSE;
        break;
    case 'Y':
        return DRAW;
        break;
    case 'Z':
        return WIN;
        break;
    }
}

int evaluatePoints(Token playerToken, Token opponentToken) {
    return playerToken + evaluationMatrix[playerToken][opponentToken] * 3 + 1;
}

//Task1
int main1(int argc, char *argv) {
    char buffer[8];
    int points = 0;
    while(!feof(stdin)) {
        fgets(buffer,8,stdin);
        points += evaluatePoints(symbolToToken(buffer[2]),symbolToToken(buffer[0]));
    }
    printf("Overall points gained for the given Strategy: %d\n",points);
    return 0;
}

//Task2
int main(int argc, char *argv) {
    char buffer[8];
    int points = 0;
    while(!feof(stdin)) {
        fgets(buffer,8,stdin);
        Token opponentToken = symbolToToken(buffer[0]);
        GameResult desiredResult = symbolToResult(buffer[2]);
        Token playerMove = whichTokenToPick[opponentToken][desiredResult];
        points += evaluatePoints(playerMove, opponentToken);
    }
    printf("Overall points gained for the given Strategy: %d\n",points);
    return 0;
}