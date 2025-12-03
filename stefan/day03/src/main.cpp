#include <algorithm>
#include <cstddef>
#include <fstream>
#include <iostream>
#include <string>

using namespace std;

//deprecated -> see getJoltageFromLineManyBatteries
int getJoltageFromLineTwoBatteries(string line) {
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

long getJoltageFromLineManyBatteries(string line, int numberBatteriesNeeded) {
  string joltageAsString = "";
  auto lastFoundIndex = -1;
  for (int i = 0; i < numberBatteriesNeeded; i++) {
    auto highestValue = 0;
    auto maxIndex = line.length() - (numberBatteriesNeeded - i);
    size_t nextIndex = lastFoundIndex + 1;
    while (nextIndex <= maxIndex) {
      auto batteryValue = line[nextIndex] - '0';
      if (batteryValue > highestValue) {
        highestValue = batteryValue;
        lastFoundIndex = nextIndex;
      }
      if (highestValue == 9) {
        break;
      }
      nextIndex++;
    }
    joltageAsString += to_string(highestValue);
    highestValue = 0;
  }
  return stol(joltageAsString);
}

int main() {
  ifstream file("day03/batteries.txt");
  string line;
  long totalJoltage = 0;
  if (file.is_open()) {
    while (getline(file, line)) {
      totalJoltage += getJoltageFromLineManyBatteries(line, 12); //for part one just use 2
    }
  }
  cout << totalJoltage << endl;
  return 0;
}
