# Prob2Dices

Prob2Dices is a small command-line tool designed to generate lists of two-dice values based on input probabilities. It is particularly useful for board game developers seeking to simulate or balance dice rolls with specific probability distributions.

## Features
- Simulates rolling two dice with custom probability distributions.
- Outputs lists of dice results that fulfill the given probabilities.
- Easy to integrate into board game development workflows.

## Motivation
Board games often rely on random events like dice rolls, and balancing these events can significantly impact gameplay. Prob2Dices enables game developers to simulate and test two-dice rolls based on predefined probabilities, ensuring that mechanics are both fun and challenging.

## Installation

### 1. Clone repository:

```bash
git clone https://github.com/Gieneq/Prob2Dices.git
```

### 2. Navigate to the project directory

```bash
cd prob2dices
```

### 3. Build the program:

```bash
cargo build --release
```

### 4. Run the program:

```bash
./target/release/prob2dices
```

## CLI Arguments

* --probabilities or -p: A separated **sorted** list of probabilities for dice results. Each value corresponds to a target probability so that player has chance to roll one of values from resulting list.

## Example

For the command:

```bash
./prob2dices -p 0.1 0.2 0.8
```

Output:

```bash
Finding coverage for probabilities: [0.1, 0.2, 0.8] ...
Coverage found with deviation: 0.041573994:
Group 1: [5], prob: 0.11111111, target: 0.1
Group 2: [2, 4, 5], prob: 0.22222222, target: 0.2
Group 3: [2, 3, 4, 5, 6, 7, 8, 10, 12], prob: 0.8333334, target: 0.8
```

Coverage is hard to fit exacly, so deviation parameter is introduce. Less deviation closer overall result.

## How It Works

Brute force search with best match criteria.

## Limitations

- The program is not checking if input probabilities are sorted, should be.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.