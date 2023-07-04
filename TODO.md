# TODO

## YUE MARKETING

1. analyze Dioxus and Xilem, compare, note plus/minus, how production ready, explain 'CS trees', develop simple 'CS trees' like Xelim/Vello
2. analyze Impeller and Makepad, compare, note plus/minus, how production ready, how Adapters would render 'CS trees'
3. first PoC with basic UI design, basic 'CS trees', basic widget structure like Flutter, first few widgets
4. more widgets and examples

## RESEARCH

### DIOXUS (1)

+ clone their repo and play around with it
- play around with the user side
- try to write something like Vullow in Dioxus
- write down what it doesn't do good enough in the form of critique they can use to improve their product

### XILEM (1)

+ clone their repo and play around with it
- play around with the user side
- try to write something like Vullow in Xilem
- write down the positive aspects and what is missing, especially about speed and performance tests

### 'CS TREES' (1)

- make tiny program that uses compute shaders to render pixels (linux only)
- design very simple tree format that can be rendered with compute shaders (CS) or old-style buffered
- design Adapter trait
- figure out how to measure speed differences in different implementations of the renderer (CS or buffered)

### MAKEPAD (2)

+ clone their repo and play around with it
- play around with the low-level interfaces
- design Adapter for makepad

#### MAKEPAD FEATURE IDEAS (2)

- inside makepad, makepad inside

### IMPELLER (FLUTTER) (2)

+ clone their repo and play around with it
- play around with the low-level interfaces
- design Adapter for impeller

## PLATFORMS (3)

- size up which platforms to support and how, maybe introduce Adapter for backends other than Makepad or Impeller

## UI DESIGN STANDARD (3)

- take Material and simplify somewhat

## WIDGETS (3)

- convert Adapter trait to more final form
- design basic widget structure based on Xelim
- cut widget list up into portions that can be marketed towards Yue

## MORE WIDGETS (4)

- develop all the widgets...
