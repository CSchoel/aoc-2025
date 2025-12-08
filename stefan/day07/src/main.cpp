#include <_stdio.h>
#include <cstddef>
#include <fstream>
#include <ios>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

void getSolutionFirstProblem(ifstream &file) {
  string line;
  vector<string> manifold;
  if (file.is_open()) {
    while (getline(file, line)) {
      manifold.push_back(line);
    }
  }
  int numberSplit = 0;
  for (size_t i = 0; i < manifold.size(); i++) {
    if (i == 0) {
      string &firstLine = manifold[0];
      auto startPoint = firstLine.find_first_of('S', 0);
      manifold[1][startPoint] = '|';
      continue;
    }
    string &currentLine = manifold[i];
    for (size_t j = 0; j < currentLine.size(); j++) {
      const char currentChar = currentLine[j];
      const char topCurrentChar = manifold[i - 1][j];
      const char bottomCurrentChar = manifold[i + 1][j];
      if (currentChar == '^' && topCurrentChar == '|') {
        currentLine[j - 1] = '|';
        currentLine[j + 1] = '|';
        manifold[i + 1][j - 1] = '|';
        manifold[i + 1][j + 1] = '|';
        numberSplit++;
      } else if (currentChar == '|' && bottomCurrentChar != '^') {
        manifold[i + 1][j] = '|';
      }
    }
  }
  cout << numberSplit << endl;
}

const long UNCALCULATED = -1;

long getPosibleWays(const vector<string> &manifold, const int currentLineIndex,
                    const int myBeamPos, vector<vector<long>> &memo) {
  if (currentLineIndex >= static_cast<int>(manifold.size())) {
    return 1l;
  }
  const string &currentLine = manifold[currentLineIndex];
  if (myBeamPos < 0 || myBeamPos >= static_cast<int>(currentLine.length())) {
    return 0l;
  }
  if (memo[currentLineIndex][myBeamPos] != UNCALCULATED) {
    return memo[currentLineIndex][myBeamPos];
  }
  const int nextLineIndex = currentLineIndex + 2;
  char charInThisLineAtBeamPos = currentLine[myBeamPos];

  long result = 0;

  if (charInThisLineAtBeamPos == '.') {
    result = getPosibleWays(manifold, nextLineIndex, myBeamPos, memo);
  } else {
    result = getPosibleWays(manifold, nextLineIndex, myBeamPos - 1, memo) +
             getPosibleWays(manifold, nextLineIndex, myBeamPos + 1, memo);
  }
  return memo[currentLineIndex][myBeamPos] = result;
}

void getSolutionSecondProblem(ifstream &file) {
  string line;
  vector<string> manifold;
  if (file.is_open()) {
    while (getline(file, line)) {
      manifold.push_back(line);
    }
  }

  string::size_type startPos = manifold[2].find_first_of('^', 0);
  int maxCols = manifold[0].size();
  int rows = manifold.size();
  vector<vector<long>> memo(rows, vector<long>(maxCols, UNCALCULATED));

  long possibleWays = getPosibleWays(manifold, 2, startPos, memo);

  cout << possibleWays << endl;
}

int main() {
  ifstream file("day07/manifolds.txt");
  getSolutionFirstProblem(file);
  file.clear();
  file.seekg(0, ios::beg);
  getSolutionSecondProblem(file);
  return 0;
}
