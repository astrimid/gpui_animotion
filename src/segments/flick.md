This segment solves the differential equation for exponential velocity decay ($\dot{v} = -c v$) in closed form:

$$v(t) = v_0 e^{-c t}$$

$$x(t) = x_0 + \frac{v_0}{c} \left( 1 - e^{-c t} \right)$$

Natural settling duration is calculated from the velocity threshold $\epsilon$:

$$T_{\text{settle}} = \frac{\ln(\vert{}v_0\vert{} / \epsilon)}{c}$$

