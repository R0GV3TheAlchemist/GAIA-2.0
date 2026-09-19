# VI-EXTRAPOLATION

Claim class: experimental.
One-page map of first-order variational inequality methods cited from the alignment shelf.

Operator F monotone L-Lipschitz on convex X unless noted.
Merit: dual gap G of averages, or last residual ||F(x_k)|| on unconstrained equations.

| Method | F calls | Last-iterate (typical) | Need |
| --- | --- | --- | --- |
| MD / OMD | 1 | averages; last may orbit | convex / online |
| EG / Mirror Prox | 2 | ||F|| ~ k^{-1/2}; gap of avg O(1/k) | monotone + Lip |
| Popov / OGDA | 1 after warmup | same class as EG on static games | monotone + Lip |
| EAG / FEG | 2 (or 1 past) | ||F||^2 = O(1/k^2) | anchor; FEG allows small co-hypo |
| S-FEG | noisy F | O(1/k^2)+floor | Var ~ 1/k |
| SVRG-EG | 1 component + snapshot | linear dist to Z* | finite sum + error bound |
| Nash-MP | 2 prox in KL | linear KL to pi*_beta | beta-regularized P |

Stochastic Mirror-Prox expected gap O(L/K + sigma/sqrt(K)).
Inexact inner prox adds a ball; sloppy REINFORCE is not Theorem 1.

Refuse runtime: no crate named after these methods until a test names the merit and the frozen oracle.
