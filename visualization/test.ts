import { linearize, merge } from './main'


/*
console.log(linearize({
    'A': ['B', 'C'],
    'B': [],
    'C': ['D'],
    'D': []
}));
*/

console.log(merge(
    [['B'], ['C','D']]
));