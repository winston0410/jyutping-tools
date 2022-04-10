# Notes on ML

## Essential ML jargon

### Parameter vs Hyperparameter

- **parameter** are required by the model to make predication. They are not manually set, and are learnt/estimated from training data

- **hyperparameter** are external to the model, and its value cannot be predicated from data. This parameter will have to be defined manually.

### Label vs feature

- **label** is the thing that we are predicting in **simple linear regression**

- **feature** is the input variable in **simple linear regression**. There could be mulitple features

### Example

- **example** is a slice/instance of whole data set

- **labelled example** is a data with **feature and label** (more like expected output?)

- **unlabelled example** contains everything except **label**

- So usually **labelled example** will be used to train the model, and **unlabelled example** will be used to test the model

### Regression vs classification model

- regression model is used for predicting **continuous value**

- classification model is used for predicting **discrete value**

### What is linear regression?

`y` is the label, `x` is the feature

```
y = ax + b
```

In ML terminology:

`w` is the weight of that feature

```
y = wx + b
```

There could be multiple features in the equation

```
y = wx₁ + wx₂ + b
```

### What is training

- to build a model that would minimize losses

- **loss** is the difference between reality and prediction

### How to reduce losses

- the resulting plot for a regression model will always be convex (bowl-shaped)

- the minimum point of the model is the point with less loss

- use **gradient descent** to move closer to minimum point without iterating all data points

- **learning rate** is the scalar(i.e. number) that **gradient descent** will multiply. It will determine the magnitude for **gradient descent** to move closer to the minimum point

- small learning rate will increase the training time, large learning rate might miss the minimum in the model

#### Optimizing learning rate

- the optimized learning rate is called **Goldilocks learning rate**

#### Optimize iteration time with gradient Descent

- Iterate all data for finding the minimum point is ineffective

- **Stochastic gradient descent** will only use **one random data** for each iteration, but there will be a lot of noise due to small sample size

- **Mini-stochastic gradient descent** will use **10-1000** data for each iteration. Less noise and still efficient

### Generative vs Discriminative model

two types of models, **Generative** and **Discriminative**

- **Generative** 

https://medium.com/@mlengineer/generative-and-discriminative-models-af5637a66a3

https://medium.com/@mlengineer/joint-probability-vs-conditional-probability-fa2d47d95c4a

### Conditonal probability

- it is a tool for quantifying **dependent events**. For calculating the propability of two **independent event**(like tossing two dices and both get 6), **joint probability** is used.

- Probability is the calculation of **desired outcome / all outcome**

- Given there are two events, and you want to know the P for A happens when B has happened, the propability will be `( P(A & B) / P(B))`. `P(B)` represents all outcome, as **B has happened**

https://mithunmanohar.medium.com/machine-learning-101-what-the-is-a-conditional-probability-f0f9a9ec6cda#:~:text=The%20conditional%20probability%20of%20an,(%20B%20)%20has%20already%20occurred.

### Bayes Theorm, Prior & Posterior Probability

- Bayes Theorm is used to calculate **the Posterior Probability**

- The **posterior probability** is the product of **prior probability** and **likelihood ratio**

- **Posterior probability** `P(H|E)` is the P which **hypothesis happens after getting evidence**

- **Prior probaility** `P(H)` is the P which **hypothesis happens before getting evidence**

- After adjustment with **likelihood ratio**, **Prior probability** will equal **Posterior probability**

https://towardsdatascience.com/understand-bayes-rule-likelihood-prior-and-posterior-34eae0f378c5

https://alexanderetz.com/2015/04/15/understanding-bayes-a-look-at-the-likelihood/

### Supervised vs Unsupervised learning

- supervised learning is based on **previous output**, tagged manually, and then train the function to estimate based on those previous output

- unsupervised learning

https://towardsdatascience.com/supervised-vs-unsupervised-learning-14f68e32ea8d

### Named-entity recognition(NER)

https://medium.com/mysuperai/what-is-named-entity-recognition-ner-and-how-can-i-use-it-2b68cf6f545d

### Random tree algo?

#### Real world example

https://levelup.gitconnected.com/classifying-reddit-posts-with-natural-language-processing-and-random-forest-classifier-af2d8fa77bd3

https://gitlab.com/ruivieira/random-forests

### Generalization

- generalization is the model's ability to give sensible output that the **input has not seen before**

- a good model is one that is **neither underfit or overfit**

- in general, good generalization captures **dominant trends** instead of **all trends**, otherwise it will be regarded as overfitting

### Overfitting and Underfitting

- ideally the algorithm should perform well with both **training data** and **new data**

- but usually the algorithm will perform **well with training datat but not on new data**

- **underfitting** happens when the model perform bad with **training data**/ training error is high(i.e. the model is does not fit the real problem well even with training data)

- **overfitting** happens when the model perform well with **training data** but not with **real data**/ high testing error compare with training error(i.e. the test used to train the model could be biased, making it performs better than in testing with reallife data)

### Precision and Recall

- Jargon to evaluate the performance of a model

- **precision** is the ratio of true positive to the sum of true positive and false positive

- **precision** is used to calculate how many **postive are actually correct**

- **recall** is used to calculate how many **actual positive was identitfied correctly**

- **recall** is the ratio of true positive to the sum of true positive and false negative

- A model with **no false positive has a precision of 1**, a model with **no false negative has a recall of 1**

https://developers.google.com/machine-learning/crash-course/classification/precision-and-recall