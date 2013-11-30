fn main() {

    struct MyTup(int, int, f64);
    let mytup: MyTup = MyTup(10, 20, 30.0);
    match mytup {
        MyTup(a, b, c) => println!("{}", a + b + (c as int))
    };

    struct GizmoId(int);

    let my_gizmo_id: GizmoId = GizmoId(10);
    let id_int: int = *my_gizmo_id;
    println!("{} : {}", id_int, *my_gizmo_id);

}
