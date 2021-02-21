# twilight-dispatch-andesite

This is a standalone service with similar concept as
[twilight-dispatch](https://github.com/chamburr/twilight-dispatch), meant to be used with Andesite.
It mainly serves as a middleman to manage cache with Redis, publishes and receives events to and
from RabbitMQ.

If you encounter any issues while running the service, please feel free create an issue here, or you
can contact CHamburr#2591 on Discord. We will try our best to help you.

## Features

-   Low CPU and RAM footprint
-   Resumable after restart
-   Prometheus metrics
-   State cache with Redis
-   Docker container support

## Implementation

### Events

Gateway events are forwarded to and from RabbitMQ.

Events are sent to a topic exchange `player`, with the event name as the routing key. By default,
there is a `player.recv` channel bound to all messages from the exchange. To send events to
Andesite, connect to the channel `player.send`, then publish the raw event as you normally would.

### State Cache

State caching with Redis is supported out of the box.

The objects available are in the table below. All values are stored in the plain text form.

| Key               | Description                     |
| ----------------- | ------------------------------- |
| `player:guild_id` | Player information for a guild. |
| `player_id`       | Andesite connection ID.         |
| `player_stats`    | Andesite statistics.            |

## Installing

The installation steps are similar to that of twilight-dispatch. Please refer to that
[here](https://github.com/chamburr/twilight-dispatch#installing) instead.

## License

This project is licensed under [ISC](LICENSE).
