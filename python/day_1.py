#!/usr/bin/env python
def get_contents(path):
  with open(file_path, 'r') as f:
    contents = [int(line) for line in f]
  return contents

def part_1(numbers):
  for a in range(len(numbers)-1):
    for b in range(a, len(numbers)):
      if numbers[a]+numbers[b] == 2020:
        return numbers[a] * numbers[b]
  return 1

def part_2(numbers):
  for a in range(len(numbers)-2):
    for b in range(a, len(numbers)-1):
      for c in range(b, len(numbers)):
        if numbers[a]+numbers[b]+numbers[c] == 2020:
          return numbers[a] * numbers[b] * numbers[c]
  return 1

if __name__ == "__main__":
  import sys

  if len(sys.argv) < 2:
    print("Missing input path in argv")
    sys.exit(1)
  file_path = sys.argv[1]
  contents = get_contents(file_path)
  
  print("Part 1:", part_1(contents))
  print("Part 2:", part_2(contents))
