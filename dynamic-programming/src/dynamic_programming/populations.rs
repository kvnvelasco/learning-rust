/*
There are two variations of this problem. Immortal and mortal populations.
Given
* a family size (number of children) (X)
* a time (years) to sexual maturity (M)
* and (optionally) a life span in years (D)

one can compute the population of beings after N years
*/

/*
State modeling:
The next number of beings is dependent on the number of sexually mature
beings (beings born M years ago) * X.

The next number of beings is next beings + current beings - beings born D years ago.
The big question is weather or not beings due to die this tick still produce offspring
*/



#[cfg(test)]
mod tests {

}