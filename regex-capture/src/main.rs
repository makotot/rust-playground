use regex::Regex;

const TEXT: &'static str = "
import { useState, useReducer } from 'react'
import { createApi } from '@reduxjs/toolkit/query'
import ReactDOM from 'react-dom'
import {
  foo
} from 'foo'
import {
    bar as b
} from 'bar/bar'
";

fn main() {
    let re = Regex::new(r"import\s+?(?:(?:(?:[\w*\s{},]*)\s+from\s+?)|)(?:(?:'.*?')|(?:'.*?'))[\s]*?(?:;|$|)").unwrap();

    for caps in re.captures_iter(TEXT) {
        println!("{}", caps.get(0).unwrap().as_str());
    }
}
