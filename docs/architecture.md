


1. separate Front end and Backend

    Flutter+ Rust   API    Backend
    
1.1 research model of key management 


2. Set proper storage (rocksdb/sled)






milestones
    easy to non-coder to set entire
    demonstrate landing page calling flows


Use cases
    music NFT
        release album as NFTs
        monitor, distribute royalties to NFT holder, staking?
        event ticket release, scan, and in-concert payment/special access
        issue rewards to certain fans



IndraDB
    create a root graph with root_graph_id
        - create new flow: create a node with some graph_id(workspace/canvas)
                -create_node: graphid -> edge -> node


IndraDB -> Model -> State > View > Flutter


basic commands
    print
    json extract
    http

solana commands <- solana context


**What is stored in db:**
graph / flow or canvas
    node

stored as nodes in indra
    solana context / one per graph

    block
    command
    input/output

    bookmarks

stored as edges in indra
    node edges
    flow edges


Block  -node edges parent of>     TextInput
                                   Command
                                        input/output nodes     flow edges




Click deploy on UI
    Read the graph, creates tx/rx on each node, send to Tokio
    Create a clone of the graph for logging purposes ->


