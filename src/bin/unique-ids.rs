use challenge::node::{
    main_loop,
    unique_ids::{UniqueIdNode, UniqueIdPayload},
};

fn main() -> anyhow::Result<()> {
    main_loop::<UniqueIdPayload, UniqueIdNode>()
}
