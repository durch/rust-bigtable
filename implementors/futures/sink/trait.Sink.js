(function() {var implementors = {};
implementors["futures"] = [];
implementors["hyper"] = [{text:"impl <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"hyper/body/struct.Sender.html\" title=\"struct hyper::body::Sender\">Sender</a>",synthetic:false,types:["hyper::body::body::Sender"]},];
implementors["tokio_sync"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_sync/mpsc/struct.Sender.html\" title=\"struct tokio_sync::mpsc::Sender\">Sender</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::mpsc::bounded::Sender"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_sync/mpsc/struct.UnboundedSender.html\" title=\"struct tokio_sync::mpsc::UnboundedSender\">UnboundedSender</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::mpsc::unbounded::UnboundedSender"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/sink/trait.Sink.html\" title=\"trait futures::sink::Sink\">Sink</a> for <a class=\"struct\" href=\"tokio_sync/watch/struct.Sender.html\" title=\"struct tokio_sync::watch::Sender\">Sender</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::watch::Sender"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()