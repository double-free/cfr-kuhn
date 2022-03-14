# Counterfactual Regret Minimization on Kuhn Poker

Implementation of CFR and MCCFR (outcome-sampling) with rust.


## Verify Result

- The expected payoff for the first player shall be -1/18

- Probability of betting with hand card 3 is 3x of with card 1

- Always pass with hand card 2


## Example Result

Here's the example output for 100000 iterations.

### CFR

| History | Check Probability | Bet Probability |
| :-: |  :-: |  :-: |
| 1 | 0.77 | 0.23 |
| 1C | 0.67 | 0.33 |
| 1B | 1.00 | 0.00 |
| 1CB | 1.00 | 0.00 |
| 2 | 1.00 | 0.00 |
| 2C | 1.00 | 0.00 |
| 2B | 0.66 | 0.34 |
| 2CB | 0.42 | 0.58 |
| 3 | 0.30 | 0.70 |
| 3C | 0.00 | 1.00 |
| 3B | 0.00 | 1.00 |
| 3CB | 0.00 | 1.00 |


### MCCFR

Explore with `epsilon=0.06`. Average payoff = -0.05138

| History | Check Probability | Bet Probability |
| :-: |  :-: |  :-: |
| 1 | 0.805 | 0.195 |
| 1C | 0.68 | 0.32 |
| 1B | 0.97 | 0.03 |
| 1CB | 0.97 | 0.03 |
| 2 | 0.97 | 0.03 |
| 2C | 0.94 | 0.05 |
| 2B | 0.56 | 0.44 |
| 2CB | 0.47 | 0.53 |
| 3 | 0.422 | 0.578 |
| 3C | 0.03 | 0.97 |
| 3B | 0.03 | 0.97 |
| 3CB | 0.03 | 0.97 |

We get 0.03 here because we choose `epsilon = 0.06` and we have 2 actions to choose from.
