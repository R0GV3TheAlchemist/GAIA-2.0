# STOCHASTIC-VI

Claim class: experimental.

S-SEG: same xi on both EG legs. Default.
I-SEG: two independent samples; can diverge on bilinear games.
SMP: Bregman SEG; E G(bar y) = O(L/K + sigma/sqrt(K)).
S-Popov-MP: one new sample + stale slope; O(sigma/sqrt(T)) on weighted averages; reuse != VR.
S-FEG: last ||F||^2 = O(1/k^2)+floor iff Var = O(1/k). Not SVRG.
SVRG-EG: snapshot estimator, same i both legs. Averages O(1/T). Last-iterate linear dist(Z*) under error bound. Needs frozen finite sum.
SAGA: table of last grads; O(nd) RAM; same linear class as SVRG on finite-sum min.
Growing batch is not VR.

Refuse: live reward model as finite-sum SVRG. Refuse S-FEG 1/k^2 on persistent sigma.
Same-sample first, then pick snapshot/table/batch.
