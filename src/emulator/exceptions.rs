#[derive(thiserror::Error, Debug)]
pub enum ExceptionHandlerErr {}

#[derive(thiserror::Error, Debug)]
pub enum Exceptions {
    #[error("Instruction address misaligned: {0}")]
    ExceptionInstructionAddressMisaligned(usize),
    
    #[error("[Placeholder]: {0}")]
    ExceptionAccessFault(usize),

    #[error("[Placeholder]: {0}")]
    ExceptionPageFault(usize),
}


//do not call this directly, instead use emu.handle_exception 
pub fn handle_expection(exception: Exceptions) -> Result<bool, ExceptionHandlerErr> {
    let continue_execution = false;

    todo!("exceptions");

    Ok(continue_execution)
}


/*
    1.6. Exceptions, Traps, and Interrupts

        Exception:  unusual condition at runtime caused by current instruction
        Interrupt:  external asynchronous event that may cause a unexpected transfer of control
        Trap:       refers to the transfer of control to a trap handler caused by either an exception or an interrupt
        
        Type of traps from the perspective of software running inside an execution environment: 
            Contained Trap: The trap is visible to, and handled by, software running inside the execution environment.
            Requested Trap: The trap is a synchronous exception that is an explicit call to the execution environment requesting an action on behalf of software inside the execution environment.
            Invisible Trap: The trap is handled transparently by the execution environment and execution resumes normally after the trap is handled.
            Fatal Trap:     The trap represents a fatal failure and causes the execution environment to terminate execution.
            
            
            Table 1. Characteristics of traps

            +-----------------------------------+---------+---------+---------+---------+
            |           Characteristics         | Contained | Requested | Invisible | Fatal     |
            +-----------------------------------+---------+---------+---------+---------+   
            | Execution terminates              |    No    |     No(1) |     No    |   Yes      |
            +-----------------------------------+---------+---------+---------+---------+   
            | Software is oblivious             |    No    |     No    |    Yes    |   Yes(2)   |
            +-----------------------------------+---------+---------+---------+---------+   
            | Handled by environment            |    No    |    Yes    |    Yes    |   Yes      |
            +-----------------------------------+---------+---------+---------+---------+

            1. Termination may be requested
            2. Imprecise fatal traps might be observable by software


            

            
*/

