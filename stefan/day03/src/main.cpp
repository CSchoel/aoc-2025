#include <cstddef>
#include <fstream>
#include <iostream>
#include <string>

using namespace std;

int getJoltageFromLine(string line) {
  int firstValue = 0;
  int firstValueIndex = 0;
  int secondValue = 0;
  for (size_t i = 0; i < line.length() - 1; i++) {
    auto batteryValue = line[i] - '0';
    if (batteryValue == 9) {
      firstValue = 9;
      firstValueIndex = i;
      break;
    } else if (batteryValue > firstValue) {
      firstValue = batteryValue;
      firstValueIndex = i;
    }
  }
  for (size_t i = firstValueIndex + 1; i < line.length(); i++) {
    auto batteryValue = line[i] - '0';
    if (batteryValue == 9) {
      secondValue = 9;
      break;
    } else if (batteryValue > secondValue) {
      secondValue = batteryValue;
    }
  }
  string joltageAsString = to_string(firstValue) + to_string(secondValue);
  return stoi(joltageAsString);
}

int main() {
  ifstream file("day03/batteries.txt");
  string line;
  int totalJoltage = 0;
  if (file.is_open()) {
    while (getline(file, line)) {
      totalJoltage += getJoltageFromLine(line);
    }
  }
  cout << totalJoltage << endl;
  return 0;
}
