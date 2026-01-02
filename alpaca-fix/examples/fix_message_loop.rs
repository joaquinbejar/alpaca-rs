//! # FIX Message Loop
//!
//! This example demonstrates how to receive and process FIX messages
//! in a continuous loop using the Alpaca FIX client.
//!
//! ## Prerequisites
//!
//! Set environment variables:
//! - `ALPACA_API_KEY`: Your Alpaca API key
//! - `ALPACA_API_SECRET`: Your Alpaca secret key
//!
//! ## Usage
//!
//! ```bash
//! cargo run -p alpaca-fix --example fix_message_loop
//! ```
//!
//! **Note**: This example demonstrates the API structure. FIX connections
//! require special credentials and server access.

fn main() {
    println!("=== FIX Message Loop ===\n");

    // Message loop overview
    println!("--- Message Loop Overview ---");
    println!("  The message loop continuously receives and processes");
    println!("  incoming FIX messages from the server.");

    // Basic message loop
    println!("\n--- Basic Message Loop ---");
    println!("  loop {{");
    println!("      match client.next_message().await {{");
    println!("          Ok(msg) => {{");
    println!("              // Process the message");
    println!("              client.process_message(&msg).await?;");
    println!("          }}");
    println!("          Err(e) => {{");
    println!("              eprintln!(\"Error receiving message: {{}}\", e);");
    println!("              break;");
    println!("          }}");
    println!("      }}");
    println!("  }}");

    // Message type dispatch
    println!("\n--- Message Type Dispatch ---");
    println!("  loop {{");
    println!("      let msg = client.next_message().await?;");
    println!("      ");
    println!("      if let Some(msg_type) = msg.msg_type() {{");
    println!("          match msg_type {{");
    println!("              \"0\" => println!(\"Heartbeat\"),");
    println!("              \"1\" => println!(\"TestRequest\"),");
    println!("              \"5\" => {{");
    println!("                  println!(\"Logout received\");");
    println!("                  break;");
    println!("              }}");
    println!("              \"8\" => {{");
    println!("                  let report = client.parse_execution_report(&msg)?;");
    println!("                  handle_execution_report(report);");
    println!("              }}");
    println!("              \"9\" => println!(\"OrderCancelReject\"),");
    println!("              _ => println!(\"Unknown: {{}}\", msg_type),");
    println!("          }}");
    println!("      }}");
    println!("  }}");

    // Common message types
    println!("\n--- Common Message Types ---");
    println!("  0: Heartbeat");
    println!("  1: TestRequest");
    println!("  2: ResendRequest");
    println!("  3: Reject");
    println!("  4: SequenceReset");
    println!("  5: Logout");
    println!("  8: ExecutionReport");
    println!("  9: OrderCancelReject");
    println!("  A: Logon");
    println!("  D: NewOrderSingle");
    println!("  F: OrderCancelRequest");
    println!("  G: OrderCancelReplaceRequest");

    // Async message handling
    println!("\n--- Async Message Handling ---");
    println!("  // Use tokio::select! for multiple async operations");
    println!("  loop {{");
    println!("      tokio::select! {{");
    println!("          msg = client.next_message() => {{");
    println!("              match msg {{");
    println!("                  Ok(m) => process_message(m).await,");
    println!("                  Err(_) => break,");
    println!("              }}");
    println!("          }}");
    println!("          _ = shutdown_signal.recv() => {{");
    println!("              println!(\"Shutdown requested\");");
    println!("              break;");
    println!("          }}");
    println!("      }}");
    println!("  }}");

    // Error handling in loop
    println!("\n--- Error Handling ---");
    println!("  let mut consecutive_errors = 0;");
    println!("  loop {{");
    println!("      match client.next_message().await {{");
    println!("          Ok(msg) => {{");
    println!("              consecutive_errors = 0;");
    println!("              // Process message...");
    println!("          }}");
    println!("          Err(e) => {{");
    println!("              consecutive_errors += 1;");
    println!("              if consecutive_errors > 5 {{");
    println!("                  eprintln!(\"Too many errors, exiting\");");
    println!("                  break;");
    println!("              }}");
    println!("          }}");
    println!("      }}");
    println!("  }}");

    // Graceful shutdown
    println!("\n--- Graceful Shutdown ---");
    println!("  // Handle Ctrl+C");
    println!("  let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);");
    println!("  ");
    println!("  tokio::spawn(async move {{");
    println!("      tokio::signal::ctrl_c().await.unwrap();");
    println!("      let _ = shutdown_tx.send(()).await;");
    println!("  }});");
    println!("  ");
    println!("  // In message loop, check for shutdown");
    println!("  // Then disconnect gracefully");
    println!("  client.disconnect().await?;");

    // Message processing patterns
    println!("\n--- Message Processing Patterns ---");
    println!("  1. Synchronous: Process each message before next");
    println!("  2. Channel-based: Send to worker tasks");
    println!("  3. Event-driven: Emit events for handlers");

    // Best practices
    println!("\n--- Best Practices ---");
    println!("1. Always process session-level messages");
    println!("2. Handle errors without crashing the loop");
    println!("3. Implement graceful shutdown");
    println!("4. Log all incoming messages");
    println!("5. Monitor message processing latency");

    println!("\n=== Example Complete ===");
}
