# BREGMAN

Claim class: experimental.
Listed-only. No crate.

D_h(x,y) = h(x)-h(y)-<grad h(y), x-y>. Not a metric. Dual coord theta = grad h(x). Step: theta -= eta g; x = grad h*(theta).
Three-point identity is the MD/MP descent lemma.
Hessian metric at y is grad^2 h(y). Relative smoothness of f w.r.t. h is Lip of grad f in that metric.

Prox: P_z(eta v; g) = argmin eta(<v,x>+g(x)) + D_h(x,z).
Entropy + beta KL(.||pi_ref) is the Nash inner map. Euclidean recovers ISTA when g is prox-friendly.
Moreau decomposition does not copy to KL.

Refuse: D_h as virtue. Geometry names the step, not the operator F.
