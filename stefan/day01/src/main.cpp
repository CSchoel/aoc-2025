#include <fstream>
#include <iostream>
#include <string>

using namespace std;

int main() {
  ifstream file("day01/rotations.txt");
  string line;
  int dialPosition = 50;
  int nullCounter = 0;
  if (file.is_open()) {
    while (getline(file, line)) {
      if (line.empty()) {
        continue;
      }
      auto direction = line.substr(0, 1);
      auto steps = line.substr(1, line.length() - 1);
      auto stepCount = stoi(steps);
      if (direction == "L") {
        dialPosition -= stepCount;
      } else if (direction == "R") {
        dialPosition += stepCount;
      }
      if (dialPosition < 0) {
        dialPosition = dialPosition % -100;
      } else if (dialPosition > 99) {
        dialPosition = dialPosition % 100;
      }
      if (dialPosition == 0) {
        nullCounter++;
      }
    }
    file.close();
  } else {
    cerr << "Unable to open file" << endl;
  }
  cout << "The password is: " << nullCounter << endl;
  return 0;
}
