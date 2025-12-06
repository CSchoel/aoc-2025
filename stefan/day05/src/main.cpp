#include <algorithm>
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

void getAllPossibleFreshIds(vector<Range> ranges) {
  long finalCountRanges = 0;
  for (Range range : ranges) {
    auto countRanges = (range.end_val - range.start_val) + 1;
    finalCountRanges += countRanges;
  }
  cout << finalCountRanges << endl;
}

Range getRangeFromLine(string line) {
  sregex_token_iterator ir(line.begin(), line.end(), range_delimiter, -1);
  long start_val = stol(*ir);
  ++ir;
  long end_val = stol(*ir);
  return {start_val, end_val};
}

bool compareRanges(const Range& a, const Range& b) {
  if (a.start_val != b.start_val) {
    return a.start_val < b.start_val;
  }
  return a.end_val < b.end_val;
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
    sort(ranges.begin(), ranges.end(), compareRanges);
    vector<Range> merged;
    merged.push_back(ranges[0]);

    for(size_t i = 1; i < ranges.size(); i++) {
      Range& current = merged.back();
      if (ranges[i].start_val <= current.end_val + 1) {
        current.end_val = max(current.end_val, ranges[i].end_val);
      } else {
        merged.push_back(ranges[i]);
      }
    }
    findFreshIngredients(merged, ids);
    getAllPossibleFreshIds(merged);
  }
  return 0;
}
