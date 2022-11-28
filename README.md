<img src="https://raw.githubusercontent.com/CatchemAl/metal-doddle/main/images/MetalDoddleLogo.png" width="420">

# metal-doddle
An lightweight Doddle implementation in rust


## Algorithm
Doddle offers two choices of algorithms for solving Wordle: Minimax and Entropy.

### Minimax
By default, Doddle uses a [minimax](https://en.wikipedia.org/wiki/Minimax) algorithm to solve the game. The easiest way to explain the algorithm is through example. 

Suppose you are half way through a game and have narrowed the solution down to one of four possibilties: `SKILL`, `SPILL`, `SWILL`, `STILL`.

Clearly, if we work our way through these words sequentially, the worst case scenario will be a further four guesses. To make things precise, let's create a histogram of all the scores that Wordle could return for each guess. We will consider the case where we naïvely choose the word `SKILL`.

| Guess   | Score        | Partition Size | Possible Words               |
|---------|--------------|----------------|------------------------------|
| `SKILL` | 🟩🟨🟩🟩🟩 |             3 | { `SPILL`, `SWILL`, `STILL` } |
| `SKILL` | 🟩🟩🟩🟩🟩 |             1 | { `SKILL` }                   |

The histogram is a great way to see how any guess **partitions** the remaining words. In the case above, there are two partitions with the worst case scenario being three (because three is the size of the largest partition).

Minimax works by considering all possible words in the dictionary and choosing the word that minimises the size of its largest partition. So, in searching through all possible words, minimax would stumble upon a word like 💥 `KAPOW` 💥.

| Guess   | Score        | Partition Size | Possible Words      |
|---------|--------------|----------------|---------------------|
| `KAPOW` | ⬜⬜⬜⬜⬜ |             1 | { `STILL` }         |
| `KAPOW` | 🟨⬜⬜⬜⬜ |             1 | { `SKILL` }         |
| `KAPOW` | ⬜⬜🟨⬜⬜ |             1 | { `SPILL` }         |
| `KAPOW` | ⬜⬜⬜⬜🟨 |             1 | { `SWILL` }         |

In this case, each word is partitioned perfectly into its own bucket of length one and the game can be immediately solved on the next move. It's simple enough to compute this histogram for every possible word and the approach generalises all the way through the game.

### Entropy
As an alternative to minimax, it is possible to play the game using an entropy based approach. Here, the solver always chooses the word that, on average, lowers the Shannon entropy of the game. To see how this works, let's assume we have reduced the game down to 20 possible words and decide to play the (excellent) move `THURL`. We shall construct a histogram as before - they're very useful.


| Guess   | Score        | Partition Size | Probability | Possible Words                                 |
|---------|--------------|----------------|-------------|------------------------------------------------|
| `THURL` | ⬜⬜⬜⬜⬜ |             3 |        0.15 | { `SNAKE`, `SPACE`, `SPADE` }                   |
| `THURL` | ⬜⬜⬜⬜🟨 |             1 |        0.05 | { `SCALE` }                                     |
| `THURL` | ⬜⬜⬜🟩⬜ |             3 |        0.15 | { `SCARE`, `SNARE`, `SPARE` }                   |
| `THURL` | ⬜🟩⬜⬜⬜ |             5 |        0.25 | { `SHADE`, `SHAKE`, `SHAME`, `SHAPE`, `SHAVE` } |
| `THURL` | ⬜🟩⬜⬜🟨 |             1 |        0.05 | { `SHALE` }                                     |
| `THURL` | ⬜🟩⬜🟩⬜ |             2 |        0.10 | { `SHARE`, `SHARK` }                            |
| `THURL` | 🟨⬜⬜⬜⬜ |             3 |        0.15 | { `SKATE`, `STAGE`, `STAVE` }                   |
| `THURL` | 🟨⬜⬜⬜🟨 |             2 |        0.10 | { `SLATE`, `STALE` }                            |

Under minimax, we would simply look at the largest bucket and assign a score of 5 to the word `THURL`. However, with an entropy based approach, we take into consideration how much each guess cuts down the entire problem *on average*. To do that, we need to look at all possible outcomes in the histogram and calculate the expected value of the number of bits of entropy that each guess provides. 

The probability of any outcome is calculated simply as the **Partition Size** / **Total Number of Words**. The number of bits associated with any outcome is calculates as -log(probability, base=2) and, hence, the expected number of bits is simply the sum of the bits multiplied by their respective probabilities.

In the example above, the expected number of Shannon bits is 2.83 which tells us that the guess `THURL` roughly cuts the problem size in half 2.83 times. To be explicit, cutting the problem in half once would leave 10 words left to search on average. Cutting the problem in half twice would leave 5. And cutting the problem in half 2.83 times would leave 2.82 words on average which looks eminently sensible when we look at the partition sizes remaining.

The guess with the highest information content, as measured in Shannon bits, is picked. In this case, `THURL` is pretty optimal.
