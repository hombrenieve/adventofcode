#include <functional>
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <numeric>
#include <map>

constexpr int BUFFER_SIZE = 2048;

// for string delimiter
std::vector<std::string> split(const std::string& s, std::string delimiter) {
  size_t pos_start = 0, pos_end, delim_len = delimiter.length();
  std::string token;
  std::vector<std::string> res;

  while ((pos_end = s.find(delimiter, pos_start)) != std::string::npos) {
    token = s.substr(pos_start, pos_end - pos_start);
    pos_start = pos_end + delim_len;
    res.push_back(token);
  }

  res.push_back(s.substr(pos_start));
  return res;
}
std::vector<std::string> split_with_rep(const std::string& s, const char delimiter) {
  size_t pos_start = 0, pos_end = 0;
  std::vector<std::string> res;

  do {
    pos_start = pos_end;
    while (pos_start < s.length() && s[pos_start] == delimiter) {
      pos_start++;
    }
    pos_end = pos_start;

    while(pos_end < s.length() && s[pos_end] != delimiter) {
      pos_end++;
    }

    if(pos_start != pos_end) {
      res.push_back(s.substr(pos_start, pos_end - pos_start));
    }
  } while (pos_start != s.length() && pos_end != s.length());
  
  return res;
}

void do_each_input_until_empty(std::function<void(const std::string &)> func) {
  char buffer[BUFFER_SIZE];
  while (std::cin.getline(buffer, BUFFER_SIZE - 1)) {
    if (std::cin.gcount() == 1) {
      break;
    }
    func(std::string(buffer).substr(0, std::cin.gcount() - 1));
  }
}


void do_each_input(std::function<void(const std::string &)> func) {
  char buffer[BUFFER_SIZE];
  while (std::cin.getline(buffer, BUFFER_SIZE - 1)) {
    func(std::string(buffer).substr(0, std::cin.gcount() - 1));
  }
}

std::string get_line() {
  char buffer[BUFFER_SIZE];
  if(std::cin.getline(buffer, BUFFER_SIZE - 1)) {
    return std::string(buffer).substr(0, std::cin.gcount() - 1);
  } else {
    throw std::runtime_error("no more lines");
  }
}

template <typename T>
bool meet_condition_all(const std::vector<T> &vec, std::function<bool(const T &)> func) {
  for (const auto &v : vec) {
    if (!func(v)) {
      return false;
    }
  }
  return true;
}