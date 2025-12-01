#include <fstream>
#include <iostream>
#include <string>
using namespace std;

int main() {
  ifstream file("day01/rotations.txt");
  string line;
  // int dialStart = 50;

  if (file.is_open()) {
    while (getline(file, line)) {
      cout << line << endl;
    }
    file.close();
  } else {
    cerr << "Unable to open file" << endl;
  }
  return 0;
}
