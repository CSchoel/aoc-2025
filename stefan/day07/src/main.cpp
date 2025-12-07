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
    // firstline
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
      const char bottomCurrentChar = manifold[i+1][j];
      if (currentChar == '^' && topCurrentChar == '|') {
        currentLine[j - 1] = '|';
        currentLine[j + 1] = '|';
        manifold[i + 1][j - 1] = '|';
        manifold[i + 1][j + 1] = '|';
        numberSplit++;
      } else if (currentChar == '|' && bottomCurrentChar != '^') {
        manifold[i+1][j] = '|';
      }
    }
  }
  // Not needed for the solution
  // for (string s : manifold) {
  //   cout << s << endl;
  // }
  cout << numberSplit << endl;
}

int main() {
  ifstream file("day07/manifolds.txt");
  getSolutionFirstProblem(file);
  return 0;
}
