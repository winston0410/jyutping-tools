# Doc

Build and view the doc:

```sh
cargo doc --open
```

## Notes on ML

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

- **Posterior probability** `P(H|E)` is the P which **hypothesis happens after getting evidence**

- **Prior probaility** `P(H)` is the P which **hypothesis happens before getting evidence**

- After adjustment with **likelihood ratio**, **Prior probability** will equal **Posterior probability**

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

## Reference

### Simple Dict matching

https://zhuanlan.zhihu.com/p/101247005

https://youzipi.blog.csdn.net/article/details/105525351

### ML

https://medium.com/ml2vec/overview-of-conditional-random-fields-68a2a20fa541
