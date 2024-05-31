; ModuleID = 'oko'
source_filename = "oko"

%.tuple. = type {}

define i32 @fibonacci(i32 %0) {
entry:
  %1 = icmp eq i32 %0, 0
  %2 = icmp eq i32 %0, 1
  %3 = or i1 %1, %2
  br i1 %3, label %.yes, label %.endif

.yes:                                             ; preds = %entry
  ret i32 1

.endif:                                           ; preds = %entry
  %4 = alloca %.tuple.
  %5 = sub i32 %0, 1
  %6 = call i32 @fibonacci(i32 %5)
  %7 = sub i32 %0, 2
  %8 = call i32 @fibonacci(i32 %7)
  %9 = add i32 %6, %8
  ret i32 %9
}

define i32 @main() {
entry:
  %0 = call i32 @fibonacci(i32 11)
  ret i32 %0
}
