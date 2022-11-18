use iff::ifcontrol;
use spaces::spacess;

mod math;
mod exm1;
mod var;
mod guess;
mod constVars;
mod shadowing;
mod spaces;
mod tupple;
mod indices;
mod arrray;
mod functions;
mod iff;
fn main() {
  math::nomeriOperations();
  exm1::Exm_p1();
  guess::game();
  var::vaar();
  constVars::constVars();
  shadowing::shadowing();
  spaces::spacess();
  tupple::tupple();
  indices::indices();
  arrray::arrays();
  functions::funn();
  iff::ifcontrol();
}