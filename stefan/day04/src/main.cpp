#include <algorithm>
#include <cstddef>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

struct Cordinates {
  size_t i;
  size_t j;
};

void countRolls(vector<string> grid) {
  long rollCounter = 0;
  for (size_t i = 0; i < grid.size(); i++) {
    for (size_t j = 0; j < grid[i].length(); j++) {
      auto charAt = grid[i][j];
      if (charAt == '@') {
        Cordinates surroundings[8] = {{i - 1, j - 1},  // top left
                                      {i - 1, j},      // top middle
                                      {i - 1, j + 1},  // top right
                                      {i, j - 1},      // same left
                                      {i, j + 1},      // same right
                                      {i + 1, j - 1},  // bottom left
                                      {i + 1, j},      // bottom middle
                                      {i + 1, j + 1}}; // bottom right
        auto surroundingRolleCounter = 0;
        for (auto surrounding : surroundings) {
          if (surrounding.i < 0 || surrounding.j < 0 ||
              surrounding.i > grid.size() - 1 ||
              surrounding.j > grid[i].length() - 1) {
            continue;
          }
          if (grid[surrounding.i][surrounding.j] == '@') {
            surroundingRolleCounter++;
          }
        }
        if (surroundingRolleCounter < 4) {
          rollCounter++;
          // grid[i][j] = 'x';
        }
      }
    }
  }
  cout << rollCounter << endl;
}

int main() {
  ifstream file("day04/rolls.txt");
  vector<string> grid;
  string line;
  if (file.is_open()) {
    while (getline(file, line)) {
      grid.push_back(line);
    }
    file.close();
  }
  if (!grid.empty()) {
    countRolls(grid);
  }
  return 0;
}
