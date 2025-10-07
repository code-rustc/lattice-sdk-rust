# Reference
## Entities
<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">publish_entity</a>(request: Entity) -> Result<Entity, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Publish an entity for ingest into the Entities API. Entities created with this method are "owned" by the originator: other sources, 
such as the UI, may not edit or delete these entities. The server validates entities at API call time and 
returns an error if the entity is invalid.

An entity ID must be provided when calling this endpoint. If the entity referenced by the entity ID does not exist
then it will be created. Otherwise the entity will be updated. An entity will only be updated if its
provenance.sourceUpdateTime is greater than the provenance.sourceUpdateTime of the existing entity.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client.entities.publish_entity(&Entity {}, None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">get_entity</a>(entity_id: String) -> Result<Entity, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .entities
        .get_entity(&"entityId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity_id:** `String` — ID of the entity to return
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">override_entity</a>(entity_id: String, field_path: String, request: EntityOverride) -> Result<Entity, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Only fields marked with overridable can be overridden. Please refer to our documentation to see the comprehensive
list of fields that can be overridden. The entity in the request body should only have a value set on the field 
specified in the field path parameter. Field paths are rooted in the base entity object and must be represented 
using lower_snake_case. Do not include "entity" in the field path.

Note that overrides are applied in an eventually consistent manner. If multiple overrides are created 
concurrently for the same field path, the last writer wins.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .entities
        .override_entity(
            &"entityId".to_string(),
            &"mil_view.disposition".to_string(),
            &EntityOverride {},
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity_id:** `String` — The unique ID of the entity to override
    
</dd>
</dl>

<dl>
<dd>

**field_path:** `String` — fieldPath to override
    
</dd>
</dl>

<dl>
<dd>

**entity:** `Option<Entity>` 

The entity containing the overridden fields. The service will extract the overridable fields from 
the object and ignore all other fields.
    
</dd>
</dl>

<dl>
<dd>

**provenance:** `Option<Provenance>` — Additional information about the source of the override.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">remove_entity_override</a>(entity_id: String, field_path: String) -> Result<Entity, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This operation clears the override value from the specified field path on the entity.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .entities
        .remove_entity_override(
            &"entityId".to_string(),
            &"mil_view.disposition".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity_id:** `String` — The unique ID of the entity to undo an override from.
    
</dd>
</dl>

<dl>
<dd>

**field_path:** `String` — The fieldPath to clear overrides from.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">long_poll_entity_events</a>(request: EntityEventRequest) -> Result<EntityEventResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This is a long polling API that will first return all pre-existing data and then return all new data as
it becomes available. If you want to start a new polling session then open a request with an empty
'sessionToken' in the request body. The server will return a new session token in the response.
If you want to retrieve the next batch of results from an existing polling session then send the session
token you received from the server in the request body. If no new data is available then the server will
hold the connection open for up to 5 minutes. After the 5 minute timeout period, the server will close the 
connection with no results and you may resume polling with the same session token. If your session falls behind 
more than 3x the total number of entities in the environment, the server will terminate your session. 
In this case you must start a new session by sending a request with an empty session token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .entities
        .long_poll_entity_events(
            &EntityEventRequest {
                session_token: "sessionToken".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**session_token:** `String` — Long-poll session identifier. Leave empty to start a new polling session.
    
</dd>
</dl>

<dl>
<dd>

**batch_size:** `Option<i64>` — Maximum size of response batch. Defaults to 100. Must be between 1 and 2000 (inclusive).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.entities.<a href="/src/api/resources/entities/client.rs">stream_entities</a>(request: EntityStreamRequest) -> Result<Stream<Vec<u8>>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Establishes a persistent connection to stream entity events as they occur.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .entities
        .stream_entities(&EntityStreamRequest {}, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**heartbeat_interval_ms:** `Option<i64>` — at what interval to send heartbeat events, defaults to 30s.
    
</dd>
</dl>

<dl>
<dd>

**pre_existing_only:** `Option<bool>` — only stream pre-existing entities in the environment and then close the connection, defaults to false.
    
</dd>
</dl>

<dl>
<dd>

**components_to_include:** `Option<Vec<String>>` — list of components to include, leave empty to include all components.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Tasks
<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">create_task</a>(request: TaskCreation) -> Result<Task, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Submit a request to create a task and schedule it for delivery. Tasks, once delivered, will 
be asynchronously updated by their destined agent. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client.tasks.create_task(&TaskCreation {}, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `Option<String>` 

If non-empty, will set the requested Task ID, otherwise will generate a new random
GUID. Will reject if supplied Task ID does not match [A-Za-z0-9_-.]{5,36}.
    
</dd>
</dl>

<dl>
<dd>

**display_name:** `Option<String>` — Human readable display name for this Task, should be short (<100 chars).
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Longer, free form human readable description of this Task.
    
</dd>
</dl>

<dl>
<dd>

**specification:** `Option<GoogleProtobufAny>` — Full set of task parameters.
    
</dd>
</dl>

<dl>
<dd>

**author:** `Option<Principal>` 
    
</dd>
</dl>

<dl>
<dd>

**relations:** `Option<Relations>` 

Any relationships associated with this Task, such as a parent Task or an assignee
this Task is designated to for execution.
    
</dd>
</dl>

<dl>
<dd>

**is_executed_elsewhere:** `Option<bool>` 

If set, then the service will not trigger execution of this task on an agent. Useful
for when ingesting tasks from an external system that is triggering execution of tasks
on agents.
    
</dd>
</dl>

<dl>
<dd>

**initial_entities:** `Option<Vec<TaskEntity>>` 

Indicates an initial set of entities that can be used to execute an entity aware
task. For example, an entity Objective, an entity Keep In Zone, etc.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">get_task</a>(task_id: String) -> Result<Task, ApiError></code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client.tasks.get_task(&"taskId".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` — ID of task to return
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">update_task_status</a>(task_id: String, request: TaskStatusUpdate) -> Result<Task, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update the status of a task.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .tasks
        .update_task_status(&"taskId".to_string(), &TaskStatusUpdate {}, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` — ID of task to update status of
    
</dd>
</dl>

<dl>
<dd>

**status_version:** `Option<i64>` 

The status version of the task to update. This version number increments to indicate the task's 
current stage in its status lifecycle. Specifically, whenever a task's status updates, the status 
version increments by one. Any status updates received with a lower status version number than what 
is known are considered stale and ignored.
    
</dd>
</dl>

<dl>
<dd>

**new_status:** `Option<TaskStatus>` — The new status of the task.
    
</dd>
</dl>

<dl>
<dd>

**author:** `Option<Principal>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">query_tasks</a>(request: TaskQuery) -> Result<TaskQueryResults, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Query for tasks by a specified search criteria.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client.tasks.query_tasks(&TaskQuery {}, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page_token:** `Option<String>` — If set, returns results starting from the given pageToken.
    
</dd>
</dl>

<dl>
<dd>

**parent_task_id:** `Option<String>` 

If present matches Tasks with this parent Task ID.
Note: this is mutually exclusive with all other query parameters, i.e., either provide parent Task ID, or
any of the remaining parameters, but not both.
    
</dd>
</dl>

<dl>
<dd>

**status_filter:** `Option<TaskQueryStatusFilter>` 
    
</dd>
</dl>

<dl>
<dd>

**update_time_range:** `Option<TaskQueryUpdateTimeRange>` — If provided, only provides Tasks updated within the time range.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tasks.<a href="/src/api/resources/tasks/client.rs">listen_as_agent</a>(request: AgentListener) -> Result<AgentRequest, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This is a long polling API that will block until a new task is ready for delivery. If no new task is 
available then the server will hold on to your request for up to 5 minutes, after that 5 minute timeout 
period you will be expected to reinitiate a new request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client.tasks.listen_as_agent(&AgentListener {}, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**agent_selector:** `Option<EntityIdsSelector>` — Selector criteria to determine which Agent Tasks the agent receives
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Objects
<details><summary><code>client.objects.<a href="/src/api/resources/objects/client.rs">list_objects</a>(prefix: Option<Option<String>>, since_timestamp: Option<Option<String>>, page_token: Option<Option<String>>, all_objects_in_mesh: Option<Option<bool>>) -> Result<ListResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Lists objects in your environment. You can define a prefix to list a subset of your objects. If you do not set a prefix, Lattice returns all available objects. By default this endpoint will list local objects only.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use chrono::{DateTime, Utc};
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .objects
        .list_objects(
            &ListObjectsQueryRequest {
                prefix: Some("prefix".to_string()),
                since_timestamp: Some(
                    DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
                page_token: Some("pageToken".to_string()),
                all_objects_in_mesh: Some(true),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**prefix:** `Option<String>` — Filters the objects based on the specified prefix path. If no path is specified, all objects are returned.
    
</dd>
</dl>

<dl>
<dd>

**since_timestamp:** `Option<String>` — Sets the age for the oldest objects to query across the environment.
    
</dd>
</dl>

<dl>
<dd>

**page_token:** `Option<String>` — Base64 and URL-encoded cursor returned by the service to continue paging.
    
</dd>
</dl>

<dl>
<dd>

**all_objects_in_mesh:** `Option<bool>` — Lists objects across all environment nodes in a Lattice Mesh.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.objects.<a href="/src/api/resources/objects/client.rs">get_object</a>(object_path: String) -> Result<Vec<u8>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Fetches an object from your environment using the objectPath path parameter.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .objects
        .get_object(&"objectPath".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_path:** `String` — The path of the object to fetch.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.objects.<a href="/src/api/resources/objects/client.rs">delete_object</a>(object_path: String) -> Result<(), ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an object from your environment given the objectPath path parameter.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .objects
        .delete_object(&"objectPath".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_path:** `String` — The path of the object to delete.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.objects.<a href="/src/api/resources/objects/client.rs">get_object_metadata</a>(object_path: String) -> Result<(), ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns metadata for a specified object path. Use this to fetch metadata such as object size (size_bytes), its expiry time (expiry_time), or its latest update timestamp (last_updated_at).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use lattice::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = LatticeClient::new(config).expect("Failed to build client");
    client
        .objects
        .get_object_metadata(&"objectPath".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**object_path:** `String` — The path of the object to query.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>
