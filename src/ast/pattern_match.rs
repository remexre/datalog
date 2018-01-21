use Bindings;
use ast::{Name, Term, Variable};

pub fn pattern_match(
    l: &mut [Term],
    r: &mut [Term],
) -> Option<(Bindings, Bindings)> {
    assert_eq!(l.len(), r.len());
    let len = l.len();

    let mut lb = Bindings::new();
    let mut rb = Bindings::new();
    for i in 0..len {
        match (l[i].clone(), r[i].clone()) {
            (Term::Name(lv), Term::Name(rv)) => if lv != rv {
                return None;
            },
            (Term::Var(lv), Term::Name(rv)) => {
                if lb.contains_key(&lv) {
                    return None;
                }
                lb.insert(lv.clone(), rv.clone());
                apply_binding(l, lv, rv);
            }
            (Term::Name(lv), Term::Var(rv)) => {
                if rb.contains_key(&rv) {
                    return None;
                }
                rb.insert(rv.clone(), lv.clone());
                apply_binding(r, rv, lv);
            }
            (Term::Var(lv), Term::Var(rv)) => {}
        }
    }
    Some((lb, rb))
}

fn apply_binding(terms: &mut [Term], var: Variable, name: Name) {
    let var = Term::Var(var);
    for term in terms.iter_mut() {
        if *term == var {
            *term = Term::Name(name.clone());
        }
    }
}

#[test]
fn literal() {
    use ast::{Name, Variable};

    let one = Term::new("1").unwrap();
    let x = Term::new("X").unwrap();
    let y = Term::new("Y").unwrap();

    let mut l = vec![x.clone(), y.clone(), x.clone(), y.clone()];
    let mut r = vec![one.clone(), one.clone(), x.clone(), x.clone()];

    let (lb, rb) = pattern_match(&mut l, &mut r).unwrap();
    assert_eq!(l, vec![one.clone(), one.clone(), one.clone(), one.clone()]);
    assert_eq!(r, vec![one.clone(), one.clone(), one.clone(), one.clone()]);
    assert_eq!(
        lb,
        vec![
            (Variable::new("X").unwrap(), Name::new("1").unwrap()),
            (Variable::new("Y").unwrap(), Name::new("1").unwrap()),
        ].into_iter()
            .collect()
    );
    assert_eq!(
        rb,
        vec![(Variable::new("X").unwrap(), Name::new("1").unwrap())]
            .into_iter()
            .collect()
    );
}
