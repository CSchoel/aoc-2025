#include <cstddef>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>
using namespace std;

struct Range {
  long start_val;
  long end_val;
};

const regex range_delimiter("-");

void findFreshIngredients(vector<Range> ranges, vector<long> ids) {
  long freshIngredients = 0;
  for (long id : ids) {
    for (Range range : ranges) {
      if (id >= range.start_val && id <= range.end_val) {
        freshIngredients++;
        break;
      }
    }
  }
  cout << freshIngredients << endl;
}

Range getRangeFromLine(string line) {
  sregex_token_iterator ir(line.begin(), line.end(), range_delimiter, -1);
  long start_val = stol(*ir);
  ++ir;
  long end_val = stol(*ir);
  return {start_val, end_val};
}

int main() {
  ifstream file("day05/ingredient.txt");
  vector<Range> ranges;
  vector<long> ids;
  string line;
  bool allRangesFoundInFile = false;
  if (file.is_open()) {
    while (getline(file, line)) {
      if (line == "") {
        allRangesFoundInFile = true;
        continue;
      }
      if (!allRangesFoundInFile) {
        ranges.push_back(getRangeFromLine(line));
      } else {
        ids.push_back(stol(line));
      }
    }
    file.close();
  }
  if (!ranges.empty()) {
    // ToDo better merge ranges before
    findFreshIngredients(ranges, ids);
  }
  return 0;
}
