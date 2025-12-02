#include <fstream>
#include <iostream>
#include <regex>
#include <string>
#include <vector>

using namespace std;

bool invalidId(long id) {
  // cout << "Checking: " << id << endl;
  auto idAsString = to_string(id);
  regex immediateParam(R"(^(\d+)\1$)");
  smatch match;
  return regex_search(idAsString, match, immediateParam);
}

void solvePuzzleOne(string fileContent) {
  regex delimiterIds(",");
  sregex_token_iterator iteratorIds(fileContent.begin(), fileContent.end(),
                                    delimiterIds, -1);
  sregex_token_iterator end;
  long sumIds = 0;
  while (iteratorIds != end) {
    string range = *iteratorIds;
    // cout << "Checking range: " << range << endl;
    regex delimiterRange("-");
    sregex_token_iterator iteratorRange(range.begin(), range.end(),
                                        delimiterRange, -1);
    auto startValue = stol(*iteratorRange);
    ++iteratorRange;
    auto endValue = stol(*iteratorRange);
    ++iteratorRange;

    for (long i = startValue; i <= endValue; i++) {
      if (invalidId(i)) {
        sumIds += i;
      }
    }
    ++iteratorIds;
  }

  cout << sumIds << endl;
}

int main() {
  ifstream file("day02/ids.txt");
  string fileContent = "";
  if (file.is_open()) {
    getline(file, fileContent);
  }
  solvePuzzleOne(fileContent);
  return 0;
}
