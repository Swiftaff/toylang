pub fn replace_any_unknown_types(ast: &mut super::Ast) {
    let depths = ast.get_depths_vec();
    let depths_flattened = ast.get_depths_flattened(&depths);
    //dbg!(&depths_flattened);
    for el_index in depths_flattened {
        ast.elements[el_index].0 = ast.get_updated_elementinfo_with_infered_type(el_index);
    }
    //dbg!(&self.elements);
}

pub fn get_depths_vec(ast: &mut super::Ast) -> Vec<Vec<usize>> {
    // collect a vec of all children
    // from deepest block in the 'tree' to highest
    // (ordered top to bottom for block at same level)
    // and reverse order within each block
    let mut tracked_parents: Vec<usize> = vec![0];
    let mut children: Vec<usize> = ast.elements[0].1.clone();
    let mut depths: Vec<Vec<usize>> = vec![children];
    loop {
        //println!("{:?}", &tracked_parents);
        let mut next_level = vec![];
        let current_level = depths.last().unwrap().clone();
        for el_ref in current_level {
            let el = &ast.elements[el_ref];
            children = el.1.iter().cloned().rev().collect();
            next_level = vec![]
                .iter()
                .chain(&next_level)
                .chain(&children)
                .cloned()
                .collect();
            tracked_parents.push(el_ref);
        }
        if next_level.len() > 0 {
            depths.push(next_level);
        } else {
            break;
        }
        //println!("{:?}", &tracked_parents);
    }
    depths
}

pub fn get_depths_flattened(depths: &Vec<Vec<usize>>) -> Vec<usize> {
    // flattens depths from bottom (deepest) to top
    // this is so that it can be used to traverse elements in the correct order
    // to allow correcting the types from the deepest elements first
    // since higher levels may rely on type of deeper elements.
    // e.g. a higher level "+" fn with type "i64|f64" will need to be disambiguated
    // to either i64 or f64 based on the type of it's 2 child args
    // so the two child args are fixed first (if unknown)
    // then "+" fn can be determined safely
    let mut output = vec![];
    for i in (0..depths.len()).rev() {
        let level = &depths[i];
        output = vec![].iter().chain(&output).chain(level).cloned().collect();
    }
    output
}
