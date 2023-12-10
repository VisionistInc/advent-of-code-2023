import sys

FIVE_KIND = 6
FOUR_KIND = 5
FULL_HOUSE = 4
THREE_KIND = 3
TWO_PAIR = 2
PAIR = 1
HIGH_CARD = 0

with open(sys.argv[1]) as f:
    data = f.read()

lines = data.split('\n')

card_values = {'2': 2, '3': 3, '4': 4, '5' : 5, '6' : 6, '7' : 7, '8' : 8, 
               '9' : 9, 'T' : 10, 'J' : 11, 'Q' : 12, 'K' : 13, 'A' : 14}

class Hand:

    def __init__(self, hand, bid, jokers=False):
        self.hand = hand
        self.bid = int(bid)
        self.score = self.score(jokers) 
    
    def print(self):
        print(self.hand, self.score, self.bid)
    
    def sorted(self):
        return ''.join(sorted(self.hand))
    
    def hasMatches(self, s, num):
        match = []
        for c in s:
            if s.count(c) == num:
                if not c in match:
                    match.append(c)
        return match
    
    def score(self, jokers=False):

        num_j = 0
        my_hand = self.hand
        if jokers:
            num_j = self.hand.count('J')
            my_hand = self.hand.replace('J','')

        m = self.hasMatches(my_hand,5)
        if len(m) > 0:
            return FIVE_KIND
        
        m = self.hasMatches(my_hand,4)
        if len(m) > 0:
            if num_j == 1:
                return FIVE_KIND
            return FOUR_KIND

        m = self.hasMatches(my_hand,3)
        if len(m) > 0:
            if num_j == 2:
                return FIVE_KIND
            if (num_j == 1):
                return FOUR_KIND
            m = self.hasMatches(my_hand,2)
            if len(m) > 0:
                return FULL_HOUSE
            return THREE_KIND

        m = self.hasMatches(my_hand,2)
        if len(m) == 2:
            if num_j == 1:
                return FULL_HOUSE
            return TWO_PAIR
        if len(m) == 1:
            if num_j == 3:
                return FIVE_KIND
            if num_j == 2:
                return FOUR_KIND
            if num_j == 1:
                return THREE_KIND
            return PAIR
        
        if num_j >= 4:
            return FIVE_KIND
        if num_j == 3:
            return FOUR_KIND
        if num_j == 2:
            return THREE_KIND
        if num_j == 1:
            return PAIR
        return HIGH_CARD


    def is_better(self, other_hand):
        if self.score > other_hand.score:
            return 1
        if self.score < other_hand.score:
            return -1
        for i in range(5):
            if card_values[self.hand[i]] > card_values[other_hand.hand[i]]:
                return 1
            if card_values[self.hand[i]] < card_values[other_hand.hand[i]]:
                return -1
        return 0

hands = []
for line in lines:
    hand, bid = line.split(' ')
    hands.append(Hand(hand, bid))

for i in range(len(hands)):
    for j in range(0, len(hands) - i - 1):

        if hands[j].is_better(hands[j+1]) == 1:
            temp = hands[j]
            hands[j] = hands[j+1]
            hands[j+1] = temp

sum = 0
for i in range(len(hands)):
    sum = sum + (hands[i].bid * (i+1))

print("Part 1:", sum)

card_values['J'] = 1
hands = []

for line in lines:
    hand, bid = line.split(' ')
    hands.append(Hand(hand, bid, jokers=True))

for i in range(len(hands)):
    for j in range(0, len(hands) - i - 1):

        if hands[j].is_better(hands[j+1]) == 1:
            temp = hands[j]
            hands[j] = hands[j+1]
            hands[j+1] = temp

sum = 0
for i in range(len(hands)):
    sum = sum + (hands[i].bid * (i+1))

print("Part 2:", sum)