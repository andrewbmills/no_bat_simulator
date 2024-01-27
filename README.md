# No Bat Simulator

Ever wonder why your favorite baseball player has no plate discipline? Is it really that hard not to swing? Let's see how they would fare if they didn't have a bat.

This project is inspired by a Jon Bois video on Barry Bonds' historic 2004 MLB season where he had a record-setting on-based percentage (OBP) of .6094.  Jon went on to entertain the question, what if Barry Bonds didn't have a bat?  Every pitch, he would simply watch.  Miraculously, Barry's OBP would still be .600+ if he never swung that year.

This got me thinking... what about the other great players?  What about players this past year?  Is there a batless wonder lurking in the league right now?

# Building and running the repo
Make sure you have a recent version of Rust installed including cargo.
```
> git clone origin https://github.com/andrewbmills/no_bat_simulator.git
> cargo init
> cargo build
> cargo run "Barry Bonds" "SFN" 2004
OBP for Barry Bonds in 2004: 0.60940033
OBP for Barry Bonds in 2004 without a bat: 0.6029173 with 372 walks and 245 strikeouts
```