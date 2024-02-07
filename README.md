# No Bat Simulator

Ever wonder why your favorite baseball player has no plate discipline? Is it really that hard not to swing? Let's see how they would fare if they didn't have a bat.

This project is inspired by a Jon Bois video on Barry Bonds' historic 2004 MLB season where he had a record-setting on-based percentage (OBP) of .6094.  Jon went on to entertain the question, what if Barry Bonds didn't have a bat?  Every pitch, he would simply watch.  Miraculously, Barry's OBP would still be .600+ if he never swung that year.  (It's a great video, check it out if you haven't: https://www.youtube.com/watch?v=JwMfT2cZGHg&ab_channel=SecretBase)

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

# Importing data from Retrosheet

Head to [Retrosheet](https://www.retrosheet.org/game.htm) and select a season from "Regular Season Event Files."  Extract the corresponding "'year'eve" directory to the project's "data" directory.  The file structure should look something like: "no_bat_simulator/data/<year>eve"

# Creating a plate discipline data file from Fan Graphs

The plate discipline statistics for this project are available at [fangraphs.com](https://www.fangraphs.com/leaders/major-league?pos=all&stats=bat&lg=all&qual=y&type=5&month=0&ind=0&team=0&rost=0&age=0&filter=&player=&startdate=&enddate=&pageitems=2000000000&season1=2023&season=2023).  Change the "Single Season" field to match your season statistics of interest and copy the entire batters table into a file called "'year'_plate_discipline.csv" inside of the "/data/'year'eve/" directory.