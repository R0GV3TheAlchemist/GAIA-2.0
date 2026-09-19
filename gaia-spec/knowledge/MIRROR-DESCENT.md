# MIRROR-DESCENT

Claim class: experimental.

Proof recipe: prox optimality vs comparator u; three-point identity; Young on <g, x_k - x_{k+1}>; sum; pick eta.
Regret: R_T <= D_h(u,x_1)/eta + (eta/2) sum ||g||_*^2. Offline convex: same on the average bar x.
Decreasing eta: prefer dual averaging / FTRL or stabilized OMD.
Last-iterate linear needs relative strong convexity (beta KL in policy space).
Composite g inside the prox = Bregman proximal gradient (mirror-C).

MP is not MD. MP is two proxes, same center, F at x then at y. Gap of average y is O(L Omega / K).
Popov-MP reuses F(y_{k-1}); one new F; does not cut variance.

Refuse last-iterate bilinear claims for plain MD.
