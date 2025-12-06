#include <_stdio.h>
#include <cstddef>
#include <fstream>
#include <ios>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
using namespace std;

void getSolutionFirstProblem(ifstream &file) {
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
}

struct ColumnInformation {
  vector<long> numbers;
  char op;
  int maxLengthNunmber;
};

void getSolutionSecondProblem(ifstream &file) {
  string line;
  vector<string> numbers;
  vector<ColumnInformation> columnInformations;
  long finalSum = 0;
  if (file.is_open()) {
    while (getline(file, line)) {
      // handle operator line
      if (line.starts_with('*') || line.starts_with('+')) {
        int whitespaces = 0;
        for (const char &c : line) {
          if (c != ' ') {
            if (columnInformations.empty()) {
              ColumnInformation ci;
              ci.op = c;
              columnInformations.push_back(ci);
            } else {
              ColumnInformation &prevCi = columnInformations.back();
              prevCi.maxLengthNunmber = whitespaces;
              whitespaces = 0;
              ColumnInformation ci;
              ci.op = c;
              columnInformations.push_back(ci);
            }
          } else {
            whitespaces++;
          }
        }
        ColumnInformation &prevCi = columnInformations.back();
        prevCi.maxLengthNunmber = whitespaces + 1;
      } else {
        numbers.push_back(line);
      }
    }
  }
  if (!numbers.empty() && !columnInformations.empty()) {
    int currentColumnInformationPosition = columnInformations.size() - 1;
    const int maxLineLengthIndex = numbers[0].size() -1;
    int charCounter = 0;

    ColumnInformation* ci =
        &columnInformations[currentColumnInformationPosition];

    for (int i = maxLineLengthIndex; i >= 0; i--) {
      if (charCounter == ci->maxLengthNunmber) {
        ci = &columnInformations[--currentColumnInformationPosition];
        charCounter = 0;
        continue;
      }
      string number;
      for (const string &row : numbers) {
        number.push_back(row[i]);
      }
      ci->numbers.push_back(stol(number));
      charCounter++;
    }
  }
  for (const ColumnInformation &ci : columnInformations) {
    long subtotal = 0;
    for (long n : ci.numbers) {
      if (ci.op == '+') {
        subtotal += n;
      } else if (ci.op == '*') {
        if (subtotal == 0) {
          subtotal = n;
        } else {
          subtotal *= n;
        }
      }
    }
    finalSum += subtotal;
  }
  cout << finalSum << endl;
}

int main() {
  ColumnInformation ci;
  ifstream file("day06/problems.txt");
  getSolutionFirstProblem(file);
  file.clear();
  file.seekg(0, ios::beg);
  getSolutionSecondProblem(file);
  return 0;
}
