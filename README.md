# NEUN Synapse - Neuromorphic Artificial Intelligence

## Neurons
Types:
 - Addition (1)
 - Subtraction (2)
 - Multiplication (3)
 - Division (4)
 - PI Multiply Add (5)
 - Sine (6)
 - cosine (7)
 - Tangent (8)
 - Absolute (9)

Every neuron works independently of every other neuron. Whenever it receives an input from a synapse, it takes the synapses weight and the value of the input, and performs
some function x, which is it's encoded function.

Everytime a neuron gets activated, it increases its exhaustion based on the strength of the signal that it received after its calculation. Every signal after calculation 
is normalized from -1 to 1.

Every neuron has a maximum and minimum value for the calculation, where the pre-normalized data is checked against both maximum and minimum. If it is either lower or greater
it then replaces the min or max values in the neuron before the normalization occurs.

If the neurons exhaustion is greater than the incoming value, the exhaustion is lowered for a 10th if the incoming value. The neuron is not activated and does not pass
on the activation to its connected synapses.

If the neurons exhaustion is lower than the incoming value, the exhaustion is increased for a 10th of the post-calculated value. The neuron is activated and passes on
the activation to its connected synapses.

Everytime a neuron performs an activation it calculates the importance of each of its synapses, based on how many times each synapse has been activated (Synapse strength).

If a synapse falls under a specific strength for the neuron it is cut and deleted from that neuron, and the neuron then performs a search for the strongest path of
connected neurons in a 3-hop distance. It calculates the maximum strength of the path and creates a new synapse with a weight of -1.

Every time a neuron deletes a synapse, it alerts the neuron that is on the other end of that synapse to trigger a "connection" check. This connection checks to make sure that
the neuron has a synapse where it is the input and a synapse where it is an output. If it does not have this, it then deletes itself and triggers a callback to the brain to
generate a new neuron somewhere well connected with a random amount of synapses. When it deletes itself it triggers the deletion if the synapses, which then alerts the neurons
on the other side of the synapses and the process continues. If the neuron that receives this alert is both an input and an output, it doesn't forward the signal to any other
neurons.

## Synapses
Every synapse has a weight, which is an minimum activation value where the synapse passes the information onto the output neuron. This weight changes every time it is activated
where it then multiplies the activation strength by the learning rate of the synapse, then it is added to the weight of the synapse.

Every synapse has a learning rate, which is a non-linear exponential of how many times it has been activated, which is (1 / 1 + e^(-(a + 1))) where a is activation amounts.

Every synapse has an importance which is 1 - learning rate. This dictates how well connected the synapse is. The closer it is to 1, the more important the connection is.

The number of activations is increased by 1 for every successful activations, and decreased by 1 for every 10 unsuccessful activations. This allows for synapses to become less
important over time if they become unused, and then recycled for new more important synapses.

## DNA
Every Neuron has a DNA code, which is laid out in the format:

DNA-TYPE CONNECTED NEURON TYPE CONNECTED NEURON TYPE ... and so on

Each Neuron is split with a 0 to signify a new neuron

So for example a Neuron DNA strand could be written as: 4436730 which would be:

Neuron type of Division
Connected to:
 - Division
 - Multiplication
 - Sine
 - Cosine
 - Multiplication

and the 0 signifies the end of the Neuron DNA.
