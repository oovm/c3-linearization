import { linearize, merge } from './main'



console.log(linearize({
    'A': ['B', 'C'],
    'B': [],
    'C': ['D'],
    'D': []
}));
