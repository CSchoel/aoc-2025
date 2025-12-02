#include <fstream>
#include <iostream>
#include <string>

using namespace std;

int main() {
  ifstream file("day01/rotations.txt");
  string line;
  int dialPosition = 50;
  int clickCounter = 0;
  if (file.is_open()) {
    while (getline(file, line)) {
      if (line.empty()) {
        continue;
      }
      auto direction = line[0];
      auto steps = stoi(line.substr(1));

      int addedClicks = 0;

      if (direction == 'L') {
        if (dialPosition == 0) {
          addedClicks = steps / 100;
        } else {
          if (steps >= dialPosition) {
            addedClicks = 1;
            int remaining = steps - dialPosition;
            addedClicks += (remaining / 100);
          }
        }
        int raw_diff = dialPosition - steps;
        dialPosition = ((raw_diff % 100) + 100) % 100;
      } else if (direction == 'R') {
        int raw_position = dialPosition + steps;
        addedClicks = (raw_position / 100);
        dialPosition = raw_position % 100;
      }
      clickCounter += addedClicks;

      // Solution for first password
      // if (direction == 'L') {
      //   dialPosition -= steps;
      // } else if (direction == 'R') {
      //   dialPosition += steps;
      // }
      // if (dialPosition < 0) {
      //   dialPosition = dialPosition % -100;
      // } else if (dialPosition > 99) {
      //   dialPosition = dialPosition % 100;
      // }
      // if (dialPosition == 0) {
      //   clickCounter++;
      // }
    }
    file.close();
  } else {
    cerr << "Unable to open file" << endl;
  }
  cout << "The password is: " << clickCounter << endl;
  return 0;
}
