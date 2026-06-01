impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        
        // ── Step 1: Make a mutable copy ──────────────────────────
        // Rust inputs are read-only by default
        // so we need mut copy to sort it
        // Same as: vector<int> cost = cost; in C++
        let mut cost = cost;

        // ── Step 2: Sort descending ───────────────────────────────
        // |a, b| is comparator (like greater<int>() in C++)
        // b.cmp(a) = descending order
        // a.cmp(b) = ascending order
        cost.sort_by(|a, b| b.cmp(a));
        // After sort: [9, 7, 6, 5, 2, 2]

        // ── Step 3: Create total variable ────────────────────────
        // let mut = changeable variable
        // Same as: int total = 0; in C
        let mut total = 0;

        // ── Step 4: Loop through every candy ─────────────────────
        // cost.iter()      = go through list one by one
        // .enumerate()     = gives index AND value together
        // i                = index  (0, 1, 2, 3...)
        // &price           = value  (9, 7, 6, 5...)
        // Same as: for(int i=0; i<cost.size(); i++) in C++
        for (i, &price) in cost.iter().enumerate() {

            // ── Step 5: Every 3rd candy is FREE ──────────────────
            // i=0 → (0+1)%3=1 → PAY
            // i=1 → (1+1)%3=2 → PAY
            // i=2 → (2+1)%3=0 → FREE ✓
            // i=3 → (3+1)%3=1 → PAY
            // i=4 → (4+1)%3=2 → PAY
            // i=5 → (5+1)%3=0 → FREE ✓
            if (i + 1) % 3 == 0 {
                continue; // skip adding to total → candy is FREE
            }

            // ── Step 6: PAY for candy ─────────────────────────────
            // Same as: total += cost[i]; in C
            total += price;
        }

        // ── Step 7: Return total ──────────────────────────────────
        // No semicolon = return value in Rust
        // Same as: return total; in C
        total
    }
}
