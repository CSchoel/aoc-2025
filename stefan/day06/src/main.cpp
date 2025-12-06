#include <cstddef>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
using namespace std;

int main() {
  fstream file("day06/problems.txt");
  string line;

  vector<vector<int>> numbers;
  vector<char> operators;

  long finalSum = 0;
  if (file.is_open()) {
    while (getline(file, line)) {
      stringstream stream(line);
      string nonWhitespace;
      vector<int> lineNumbers;
      while (stream >> nonWhitespace) {
        if (nonWhitespace == "+" || nonWhitespace == "*") {
          operators.push_back(nonWhitespace[0]);
        } else {
          lineNumbers.push_back(stoi(nonWhitespace));
        }
      }
      if (!lineNumbers.empty()) {
        numbers.push_back(lineNumbers);
      }
    }
    for (size_t i = 0; i < operators.size(); i++) {
      char op = operators[i];
      vector<int> columnNumbers;
      for (vector<int> lineNumbers : numbers) {
        columnNumbers.push_back(lineNumbers[i]);
      }
      long columnSum = 0;
      for (int number : columnNumbers) {
        if (op == '+') {
          columnSum += number;
        } else if (op == '*') {
          if (columnSum == 0) {
            columnSum = number;
          } else {
            columnSum *= number;
          }
        }
      }
      finalSum += columnSum;
    }
  }
  cout << finalSum << endl;
  return 0;
}
